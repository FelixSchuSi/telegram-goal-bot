from telegram import ParseMode, Message, Chat, Bot
from telegram.ext import (Updater, CommandHandler, MessageHandler)

def send_message(apis, title, mp4Link, chat_id=None) -> Message:
  text = f'<b><a href="{mp4Link}">{title}</a></b>'
  chat_id = chat_id or apis["chat_id"]
  return apis["bot"].send_message(chat_id=chat_id, text=text, parse_mode=ParseMode.HTML)


def send_video(apis, title, links, chat_id=None) -> Message:
  link, mp4_link = links
  try:
    chat_id = apis["chat_id"] if chat_id is None else chat_id
    message = apis["bot"].send_video(chat_id=chat_id, caption=title,
                                     video=mp4_link, timeout=500)
    return message
  except Exception as e:
    print(f'[TELEGRAM WRAPPER] Exception when sending video with title "{title}" to {chat_id}. Sending text instead.')
    print('[TELEGRAM WRAPPER]', e)
    return send_message(apis, title, link, chat_id)


def get_copy_of_message_in_comment_group(apis, message: Message):
  #print(f'message.chat {message.chat.title} {message.chat.type} {message.chat.linked_chat_id}')
  chat_id = message.chat_id
  chat: Chat = apis["bot"].get_chat(chat_id)
  #print(chat)
  comments_chat: Chat = apis["bot"].get_chat(chat.linked_chat_id)

  updater = Updater(apis["bot"].token)
  dispatcher = updater.dispatcher
  # def getComment
  # dispatcher.add_handler(callback=)
  message: Message = Message(chat=comments_chat, date=message.date, message_id=message.message_id)
  # print(message.text)
  # message.reply_html('Reply')
  updates = apis["bot"].get_updates(offset=10)
  print(f'updates: {updates}')


  return


# updater = Updater("TOKEN")
#
#     # Get the dispatcher to register handlers
#     dispatcher = updater.dispatcher
#
#     # on different commands - answer in Telegram
#     dispatcher.add_handler(CommandHandler("start", start))
#     dispatcher.add_handler(CommandHandler("help", help_command))
#
#     # on non command i.e message - echo the message on Telegram
#     dispatcher.add_handler(MessageHandler(Filters.text & ~Filters.command, echo))