from telegram_wrapper import send_message, send_video
from setup import read_secrets, setup
from aa_util import parse_title, queue_handler, send_links_with_texts
from telegram.ext import (Updater, CommandHandler)
from multiprocessing import Process, Queue

secrets = read_secrets()
TOKEN = secrets["telegram_token"]
apis = setup()
live_comments_queue = Queue()
live_comments_process = Process(target=queue_handler, args=(live_comments_queue, apis,))


# logging.basicConfig(level=logging.DEBUG,
#                     format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')


def main():
  updater = Updater(TOKEN, use_context=True)
  dp = updater.dispatcher

  def more_callback(update, context):
    is_eng = update.message.text == "/more"
    original_message = update.message.reply_to_message
    if original_message is None or update.message.reply_to_message.from_user.id != 1039434387:
      eng_text = "The /more command can only be used when replying to one of my messages."
      ger_text = "Du kannst das /mehr Kommando nur in Antworten auf meine Nachrichten verwenden."
      text = eng_text if is_eng else ger_text
      update.message.reply_text(text=text)
      return
    user_id = update.message.from_user.id
    title = original_message.caption or original_message.text
    eng_start_text = f"Searching for alternative angles of goal: \'{title}\'..."
    ger_start_text = f"Suche nach Kameraperspektiven von Tor: \'{title}\'..."
    start_text = eng_start_text if is_eng else ger_start_text
    print(f"[REPLIES]: Sending text {start_text} to {user_id}.")
    send_message(apis, start_text, '', user_id)

    links_with_texts, submission = parse_title(title, apis)
    if links_with_texts is None:
      eng_notfound_text = f"Couldn't find any alternative angles."
      ger_notfound_text = f"Es wurden keine Kameraperspektiven gefunden."
      notfound_text = eng_notfound_text if is_eng else ger_notfound_text
      send_message(apis, notfound_text, '', user_id)
      return

    send_links_with_texts(apis, links_with_texts, user_id, is_eng)

    live_comments_queue.put((submission, user_id))

  dp.add_handler(CommandHandler("more", more_callback))
  dp.add_handler(CommandHandler("mehr", more_callback))

  updater.start_polling()
  updater.idle()


if __name__ == '__main__':
  live_comments_process.start()
  main()
