from datetime import datetime
from multiprocessing import Manager
from typing import Any


class WatchList:
  def __init__(self):
    print(f"[WATCH LIST] Watchlist created")
    manager = Manager()
    self.watchlist = manager.dict({})
    self.last_expiration_check = datetime.utcnow()

  def append(self, aa_comment_id: str, bot_message_id: str) -> None:
    created_at = datetime.utcnow()
    if self.watchlist.get(aa_comment_id) is None:
      print(f"[WATCH LIST] putting aa_comment {aa_comment_id} in NEW entry for user {bot_message_id}")
      self.watchlist[aa_comment_id] = (bot_message_id, created_at)
    else:
      print(f'[WATCH LIST] aa_comment_id {aa_comment_id} is already on watchlist')

  def remove_expired_entries(self):
    # Entries older than 4 hours get removed from the watchlist.
    print(f"[WATCH LIST] Deleting expired entries. starting to loop over entries now.")
    try:
      for aa_comment_id, (bot_message_id, created_at) in self.watchlist.items():
        print('[WATCH LIST] Inspecting all registrations for comment: ', aa_comment_id)
        if (datetime.utcnow() - created_at).total_seconds() / 60 / 60 > 4:
          self.watchlist.pop(aa_comment_id)
    except Exception as e:
      print('[WATCH LIST] ' + str(e))

  def __getitem__(self, item: Any) -> Any:
    return self.watchlist[item]

  def __contains__(self, aa_comment_id: str) -> bool:
    self.last_expiration_check = datetime.utcnow()
    if aa_comment_id is None:
      return False
    item = self.watchlist.get(aa_comment_id)
    return False if item is None else True

  def __str__(self):
    return str(self.watchlist)
