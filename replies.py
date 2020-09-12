import os
import sys
from scrape import scrape_with_retries
from telegram_wrapper import send_message, send_video
from setup import read_secrets, setup
from threading import Thread
from alternative_angles import parseTitle
import telegram
from telegram.ext import (Updater, CommandHandler, MessageHandler, Filters,
                          ConversationHandler, CallbackQueryHandler)
from telegram.error import BadRequest
import logging

secrets = read_secrets()
TOKEN = secrets["telegram_token"]
# logging.basicConfig(level=logging.DEBUG,
#                     format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')

apis = setup()

def main():
    updater = Updater(TOKEN, use_context=True)
    dp = updater.dispatcher

    def more_callback(update, context):
        is_eng = update.message.text == "/more"
        original_message = update.message.reply_to_message
        if original_message == None or update.message.reply_to_message.from_user.id != 1039434387:
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
        
        linksWithTexts = parseTitle(title)
        
        for i, linkWithText in enumerate(linksWithTexts):
            print(f'parsing link {i+1} of {len(linksWithTexts)}')
            link, title = linkWithText
            print('linkWithText', linkWithText)
            mp4_link, new_title = parseLinkWithText(linkWithText, is_eng, i)
            links = (link, mp4_link)
            try:
                send_video(apis, new_title, links, user_id) if mp4_link else send_message(apis, title, link, user_id)
            except Exception as e:
                print('Error when sending this: ' + linkWithText)
                print(e)

        print(f"SUCCESS!")

    dp.add_handler(CommandHandler("more", more_callback))
    dp.add_handler(CommandHandler("mehr", more_callback))

    updater.start_polling()
    updater.idle()

def parseLinkWithText(linkWithText, is_eng, i):
    ger_no_desc = "Ohne Beschreibung"
    eng_no_desc = "No description"
    no_desc = eng_no_desc if is_eng else ger_no_desc

    link, title = linkWithText
    scraped_link = scrape_with_retries(link, title)
    string = no_desc if title == '' else title
    return (scraped_link, string)

if __name__ == '__main__':
    main()