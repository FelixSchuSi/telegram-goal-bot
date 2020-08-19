from telegram import ParseMode

def send_message(apis, title, mp4Link):
    text = f'<a href="{mp4Link}">{title}</a>'
    apis["bot"].send_message(chat_id=apis["chat_id"], text=text, parse_mode=ParseMode.HTML)

def send_video(apis, title, mp4Link):
    apis["bot"].send_video(chat_id=apis["chat_id"], caption=title,
                                    video=mp4Link, timeout=500)
