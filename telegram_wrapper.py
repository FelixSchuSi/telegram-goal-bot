from telegram import ParseMode


def send_message(apis, title, mp4Link, chat_id=None):
    text = f'<b><a href="{mp4Link}">{title}</a></b>'
    chat_id = chat_id or apis["chat_id"]
    apis["bot"].send_message(chat_id=chat_id, text=text, parse_mode=ParseMode.HTML)


def send_video(apis, title, links, chat_id=None):
    link, mp4_link = links
    try:
        chat_id = apis["chat_id"] if chat_id is None else chat_id
        apis["bot"].send_video(chat_id=chat_id, caption=title,
                               video=mp4_link, timeout=500)
    except Exception as e:
        print(f'[TELEGRAM WRAPPER] Exception when sending video with title "{title}" to {chat_id}. Sending text instead.')
        print('[TELEGRAM WRAPPER]', e)
        send_message(apis, title, link, chat_id)
