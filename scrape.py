from pyquery import PyQuery as pq

# TODO: "twitter", "imgtc", "clippituser", "instagram", "vimeo", "youtu"
def mp4Link(url):
    if 'streamja' in url:
        d = pq(url=url)('video > source')
        return d.attr('src')
    if 'streamable' in url:
        d = pq(url=url)('div > video')
        return 'http:' + d.attr('src')
    else:
        return False
