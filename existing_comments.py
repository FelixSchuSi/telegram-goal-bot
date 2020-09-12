from praw import models

def getExistingComments(submission):
    a_a_comment = getAlternativeAnglesCommentFromSubmission(submission)
    replies = a_a_comment.replies.list()
    comments = getAllRepliesFromComment(a_a_comment)
    return comments

def getAlternativeAnglesCommentFromSubmission(submission):
    tuple_thing = commentForestToLists(submission.comments.list())
    comments = tuple_thing[0]

    for comment in comments:
        if comment.author == "AutoModerator":
            return comment
    # If you have trouble finding the alternative angle comment in the future,
    # you should serach in the list of MoreComments objects!
    # The a_a_comment might not even exist yet, since it is created by a bot after
    # the post is created. You might want to call this function again in a few secs.

def getAllRepliesFromComment(commentOrMoreComments, temp_list=None):
    temp_list = [] if temp_list is None else temp_list
    if isinstance(commentOrMoreComments, models.MoreComments):
        for c in commentOrMoreComments.comments():
            getAllRepliesFromComment(c, temp_list)
    elif isinstance(commentOrMoreComments, models.Comment):
        temp_list.append(commentOrMoreComments)
        for child in commentOrMoreComments.replies.list():
            getAllRepliesFromComment(child, temp_list)
    else:
        print(f"What is this: {commentOrMoreComments}")
    return temp_list

def commentForestToLists(comment_forest):
    more_comments = []
    comments = []
    for commentOrMoreComments in comment_forest:
        if isinstance(commentOrMoreComments, models.MoreComments):
            more_comments.append(commentOrMoreComments)
        elif isinstance(commentOrMoreComments, models.Comment):
            comments.append(commentOrMoreComments)
        else:
            print(f"What is this: {commentOrMoreComments}")
    return (comments, more_comments)