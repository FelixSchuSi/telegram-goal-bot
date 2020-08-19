import os
import sys
from setup import read_secrets
from threading import Thread
from alternative_angles import parseTitle
import telegram
from telegram.ext import (Updater, CommandHandler, MessageHandler, Filters,
                          ConversationHandler, CallbackQueryHandler)
from telegram.error import BadRequest
import logging

secrets = read_secrets()
TOKEN = secrets["telegram_token"]
logging.basicConfig(level=logging.DEBUG,
                    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')

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

        title = update.message.reply_to_message.caption
        linksWithTexts = parseTitle(title)
        # text = linksToText(linksWithTexts, is_eng, update.message.from_user)
        for i, linkWithText in enumerate(linksWithTexts):
            link, title = linkWithText
            mp4_link, new_title = parseLinkWithText(linkWithText, is_eng, i)
            send_video(apis, new_title, mp4_link) if mp4_link else send_message(apis, title, link)
        print("SENDING: ", text)
        # original_message.reply_text(text=text, parse_mode=telegram.ParseMode.HTML, disable_web_page_preview=True)

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
    string = f"#{i+1}: {no_desc if title == '' else title}"
    return (scraped_link, string)

# def linksToText(links, is_eng, user):
#     ger_text = f"<b>Hier sind mehr Kameraperspektiven für dich <a href=\"tg://user?id={user.id}\">{user.name}</a>:</b>\n"
#     eng_text = f"<b>Here are more angles for you <a href=\"tg://user?id={user.id}\">{user.name}</a>:</b>\n"
#     ger_no_desc = "Ohne Beschreibung"
#     eng_no_desc = "No description"
#     text = eng_text if is_eng else ger_text
#     no_desc = eng_no_desc if is_eng else ger_no_desc
#     for i, linkWithText in enumerate(links):
#         link, title = linkWithText
#         string = f"#{i+1}: {no_desc if title == '' else title}"
#         text += f'<a href="{link}">{string}</a>\n'

    return text

if __name__ == '__main__':
    main()