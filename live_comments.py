# watch_list = []  # list of submissions
#
#
# def is_relevant(comment):
#     # Does this comment belong to one of the posts on the watchlist?
#     if list(filter(lambda x: x["post_id"] == comment.link_id, watch_list)):
#         root_comment = getAlternativeAnglesCommentFromSubmission(comment.submission)
#         # Is this comment a child of the stickied 'Mirrors / Alternate angles' comment?
#         if list(filter(lambda x: x["root_comment_id"] == root_comment.id, watch_list)):
#             return True
#     return False
#
#
# def register_submission(submission):
#     watch_list.append(submission)
