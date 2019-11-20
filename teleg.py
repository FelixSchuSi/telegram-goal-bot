import telegram
import secrets
bot = telegram.Bot(token=secrets.telegram_token)

def sendMessage(text):
    bot.send_message(chat_id=secrets.telegram_chat_id, text=text)

def sendVideo(caption, url):
    # TODO: crawl streamja links and extract direct link to the mp4 file. Then do this:
    # bot.send_video(chat_id=secrets.telegram_chat_id, caption="guck dir das Tor an!",
    # video="https://tiger.cdnja.co/v/g3/G3oe.mp4?secure=QYHStbyRfxUYqnfp4u0kNw&expires=1574289000")
    return

