from datetime import datetime
from multiprocessing import Manager


# Example of watchlist entry
# {aa_comment_id: [("telegram_user_id_1", created_at), ("telegram_user_id_2", created_at)]}

class WatchList:

  def __init__(self):
    print(f"[WATCH LIST] Watchlist created")
    manager = Manager()
    self.watchlist = manager.dict({})
    self.last_expiration_check = datetime.utcnow()

  def append(self, aa_comment_id, telegram_user_id):
    created_at = datetime.utcnow()
    if self.watchlist.get(aa_comment_id) is None:
      print(f"[WATCH LIST] putting aa_comment {aa_comment_id} in NEW entry for user {telegram_user_id}")
      self.watchlist[aa_comment_id] = [(telegram_user_id, created_at)]
    else:
      if isinstance(self.watchlist[aa_comment_id], list):
        print(f"[WATCH LIST] putting aa_comment {aa_comment_id} in EXISTING entry for user {telegram_user_id}")
        self.watchlist[aa_comment_id].append((telegram_user_id, created_at))
      else:
        print(f'[WATCH LIST] watch list item does not have the expected format: {self.watchlist[aa_comment_id]}')

  def remove_expired_entries(self):
    # Entries older than 4 hours get removed from the watchlist.
    print(f"[WATCH LIST] Deleting expired entries. starting to loop over entries now.")
    try:
      for aa_comment_id, registered_users in self.watchlist:
        for user_entry in registered_users:
          print('[WATCH LIST] Inspecting user_entry for expiration: ', str(user_entry))
          telegram_user_id, created_at = user_entry
          # if (datetime.utcnow() - created_at).total_seconds() / 60 / 60 > 4:
          if True:
            if len(registered_users) == 1:
              print(
                f"[WATCH LIST] Removing key and entries for aa_comment_id {aa_comment_id} since expired entry of user"
                f"{telegram_user_id} was the only entry.")
              self.watchlist.pop(aa_comment_id)
            elif len(registered_users) > 1:
              print(f"[WATCH LIST] Removing entry of user {telegram_user_id} for aa_comment_id {aa_comment_id}")
              registered_users.remove(user_entry)
    except Exception as e:
      print('[WATCH LIST]' + str(e))

  def get_registered_users(self, aa_comment_id):
    return self.watchlist[aa_comment_id]

  def __contains__(self, item):
    if isinstance(item, tuple):
      aa_comment_id, telegram_user_id = item
    else:
      aa_comment_id = item
      telegram_user_id = None

    self.last_expiration_check = datetime.utcnow()
    if aa_comment_id is None:
      return False

    registered_users = self.watchlist.get(aa_comment_id)
    if registered_users is None:
      return False

    if telegram_user_id is None:
      # Determine whether aa_comment_id is used as a key in the watchlist
      return True
    else:
      # Determine if the aa_comment_id, telegram_user_id combination is used in the watchlist
      for user_entry in registered_users:
        if user_entry[0] == telegram_user_id:
          return True
    return False

  def __str__(self):
    return str(self.watchlist)
