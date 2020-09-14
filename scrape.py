from pyquery import PyQuery as pq
from telegram.error import BadRequest
import time
# TODO: "imgtc", "vimeo"


def mp4_link(url):
    if 'streamja' in url:
        d = pq(url=url)('video > source')
        return d.attr('src')
    if 'streamable' in url:
        d = pq(url=url)('div > video')
        return 'https:' + d.attr('src')
    if 'clippituser' in url:
        d = pq(url=url)('#player-container')
        return d.attr('data-hd-file')
    if 'streamvi' in url:
        d = pq(url=url)('video > source')
        return d.attr('src')
    else:
        print(f'[SCRAPE ERROR] No existing routine for url: {url}')
        return False


def scrape_with_retries(url, title, retries=1):
    if retries <= 5:
        try:
            link = mp4_link(url)
            if not link: return False
            print('[SUCCESSFULLY SCRAPED]', title, link)
            return link
        except BadRequest as e:
            print('[BAD REQUEST]', title, url, str(e))
            return False
        except Exception as e:
            # In most cases the video is not processed yet. We will try again in a few secs.
            print(f'[RETRY NO {retries}]', title, url, str(e))
            time.sleep(15)
            return scrape_with_retries(url, title, retries + 1)
    else:
        print(f'[ERR AFTER {retries} RETRIES]', title, url)
        return False
