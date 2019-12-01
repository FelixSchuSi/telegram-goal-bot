from pyquery import PyQuery as pq
import asyncio

# TODO: "twitter", "imgtc", "instagram", "vimeo", "youtu"
async def mp4Link(url):
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
        return False
