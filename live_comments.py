def is_relevant(comment):
    # Does this comment belong to one of the posts on the watchlist?
    if list(filter(lambda x: x["post_id"] == comment.link_id, watch_list)):
        root_comment = findRootComment(comment)
        # Is this comment a child of the stickied 'Mirrors / Alternate angles' comment?
        if list(filter(lambda x: x["root_comment_id"] == root_comment.id, watch_list)):
            return True
    return False

def findRootComment(comment):
    if comment.parent_id.startswith("t1_"):
        return findRootComment(comment.parent())
    elif comment.parent_id.startswith("t3_"):
        return comment
    else:
        raise Exception("parent of comment was neither a comment nor a submission.")

def isCommentChildOfComment(comment, root_comment):
    if comment.parent_id.startswith("t1_"):
        # if comment.parent_id.startswith("t1_g007"): print(f"LOL! {comment.parent_id[3:]} {root_comment.id} {comment.author}")
        if comment.parent_id[3:] == root_comment.id:
            print(f"found! {comment.parent_id[3:]} {root_comment.id}")
            return True
        else:
            return isCommentChildOfComment(comment.parent(), root_comment)
    elif comment.parent_id.startswith("t3_"):
        # print(f"t3 found: {comment.parent_id[3:]}")
        # print(f"child of t3: {comment.id} target: {root_comment.id}")
        return False