def send_message(apis, submission):
    text = f'<a href="{submission.url}">{submission.title}</a>'
    apis["bot"].send_message(chat_id=apis["chat_id"], text=text, parse_mode=telegram.ParseMode.HTML)

def send_video(apis, submission, mp4Link):
    apis["bot"].send_video(chat_id=apis["chat_id"], caption=submission.title,
                                    video=mp4Link, timeout=500)
