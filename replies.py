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
# logging.basicConfig(level=logging.DEBUG,
#                     format='%(asctime)s - %(name)s - %(levelname)s - %(message)s')

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
        # TODO: Parse title currently returns unscraped links because reply_video always returns badrequest
        linksWithTexts = parseTitle(title)
        text = linksToText(linksWithTexts, is_eng)
        print("SENDING: ", text)
        original_message.reply_text(text=text, parse_mode=telegram.ParseMode.HTML)
        print(f"SUCCESS!")

    dp.add_handler(CommandHandler("more", more_callback))
    dp.add_handler(CommandHandler("mehr", more_callback))

    updater.start_polling()
    updater.idle()

def linksToText(links, is_eng):
    ger_text = "<b>Andere Kameraperspektiven:</b>\n"
    eng_text = "<b>Alternative angles:</b>\n"
    text = eng_text if is_eng else ger_text
    for linkWithText in links:
        link, title = linkWithText
        text += f'<a href="{link}">{title}</a>\n'

    return text

if __name__ == '__main__':
    main()