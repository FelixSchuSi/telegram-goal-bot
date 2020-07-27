from pyquery import PyQuery as pq
# TODO: "imgtc", "vimeo"

def mp4_link(url):
    try:
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
    except Exception:
        print(f'[SCRAPE ERROR] Exception when scraping this url: {url}')
        return False
