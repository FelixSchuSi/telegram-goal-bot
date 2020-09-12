from telegram import ParseMode

def send_message(apis, title, mp4Link, chat_id=None):
    print('sending text', chat_id)
    text = f'<b><a href="{mp4Link}">{title}</a></b>'
    chat_id = chat_id or apis["chat_id"]
    apis["bot"].send_message(chat_id=chat_id, text=text, parse_mode=ParseMode.HTML)

def send_video(apis, title, links, chat_id=None):
    link, mp4Link = links
    try:
        chat_id = apis["chat_id"] if chat_id is None else chat_id
        print(chat_id)
        apis["bot"].send_video(chat_id=chat_id, caption=title,
                                    video=mp4Link, timeout=500)
    except Exception as e:
        print(f'Exception when sending video with title "{title}" to {chat_id}. Sending text instead.')
        print(e)
        send_message(apis, title, link, chat_id)
        