from praw import models


def get_existing_comments(submission):
    a_a_comment = get_alternative_angles_comment_from_submission(submission)
    replies = a_a_comment.replies.list()
    comments = get_all_replies_from_comment(a_a_comment)
    return comments


def get_alternative_angles_comment_from_submission(submission):
    tuple_thing = comment_forest_to_lists(submission.comments.list())
    comments, more_comments = tuple_thing

    for comment in comments:
        if comment.author == "AutoModerator":
            return comment
    # If you have trouble finding the alternative angle comment in the future,
    # you should serach in the list of MoreComments objects!
    # The a_a_comment might not even exist yet, since it is created by a bot after
    # the post is created. You might want to call this function again in a few secs.


def get_all_replies_from_comment(comment_or_more_comments, temp_list=None):
    temp_list = [] if temp_list is None else temp_list
    if isinstance(comment_or_more_comments, models.MoreComments):
        for c in comment_or_more_comments.comments():
            get_all_replies_from_comment(c, temp_list)
    elif isinstance(comment_or_more_comments, models.Comment):
        temp_list.append(comment_or_more_comments)
        for child in comment_or_more_comments.replies.list():
            get_all_replies_from_comment(child, temp_list)
    else:
        print(f"What is this: {comment_or_more_comments}")
    return temp_list


def comment_forest_to_lists(comment_forest):
    more_comments = []
    comments = []
    for commentOrMoreComments in comment_forest:
        if isinstance(commentOrMoreComments, models.MoreComments):
            more_comments.append(commentOrMoreComments)
        elif isinstance(commentOrMoreComments, models.Comment):
            comments.append(commentOrMoreComments)
        else:
            print(f"What is this: {commentOrMoreComments}")
    return comments, more_comments
