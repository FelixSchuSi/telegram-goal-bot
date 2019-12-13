from pyquery import PyQuery as pq
from urllib.error import HTTPError
# TODO: "imgtc", "vimeo"

def mp4Link(url):
    if 'streamja' in url:
        d = pq(url=url)('video > source')
        return d.attr('src')
    if 'streamable' in url:
        d = pq(url=url)('div > video')
        return 'http:' + d.attr('src')
    if 'clippituser' in url:
        d = pq(url=url)('#player-container')
        return d.attr('data-hd-file')
    if 'streamvi' in url:
        d = pq(url=url)('video > source')
        return d.attr('src')
    else:
        print('No scraping routine specified for this host.')
        return False
