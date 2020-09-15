from scrape import scrape_with_retries
from telegram_wrapper import send_message, send_video
from setup import read_secrets, setup
from aa_util import parse_title, queue_handler
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

    title = update.message.reply_to_message.caption
    eng_start_text = f"Searching for alternative angles of goal: \'{title}\'..."
    ger_start_text = f"Suche nach Kameraperspektiven von Tor: \'{title}\'..."
    start_text = eng_start_text if is_eng else ger_start_text
    send_message(apis, start_text, '', user_id)

    links_with_texts, submission = parse_title(title, apis)

    for i, linkWithText in enumerate(links_with_texts):
      print(f'[EXISTING COMMENTS] parsing link {i + 1} of {len(links_with_texts)}')
      link, title = linkWithText
      print('[EXISTING COMMENTS] linkWithText', linkWithText)
      mp4_link, new_title = parse_link_with_text(linkWithText, is_eng)
      links = (link, mp4_link)
      try:
        send_video(apis, new_title, links, user_id) if mp4_link else send_message(apis, title, link, user_id)
      except Exception as e:
        print('[EXISTING COMMENTS] Error when sending this: ' + linkWithText)
        print(e)

    # submission oder root comment hier übergeben
    live_comments_queue.put((submission, user_id))

  dp.add_handler(CommandHandler("more", more_callback))
  dp.add_handler(CommandHandler("mehr", more_callback))

  updater.start_polling()
  updater.idle()


def parse_link_with_text(link_with_text, is_eng):
  ger_no_desc = "Ohne Beschreibung"
  eng_no_desc = "No description"
  no_desc = eng_no_desc if is_eng else ger_no_desc

  link, title = link_with_text
  scraped_link = scrape_with_retries(link, title)
  string = no_desc if title == '' else title
  return scraped_link, string


if __name__ == '__main__':
  live_comments_process.start()
  main()
