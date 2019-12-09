import praw
import secrets
from competition import Competition, Team
import telegram
from bcolors import bcolors

buli = [['gladbach', 'mönchengladbach', 'monchengladbach'], ['leipzig'], ['bayern', 'münchen', 'munchen', 'munich'], ['freiburg'], ['hoffenheim'], ['dortmund'], ['schalke'], ['leverkusen', 'bayer'],
        ['frankfurt'], ['wolfsburg'], ['union'], ['hertha'], ['düsseldorf'], ['dusseldorf'], ['werder', 'bremen'], ['augsburg'], ['mainz'], ['köln', 'koln', 'cologne'], ['paderborn']]

cl = [['ajax', 'amsterdam'], ['atalanta'], ['atlético', 'atletico'], ['bayer', 'leverkusen'], ['bayern', 'münchen', 'munich', 'munchen'], ['dortmund'], ['chelsea'], ['brügge', 'brugge'], ['roter', 'stern', 'belgrad', 'red', 'star'], ['dinamo', 'zagreb', 'dynamo'], ['barcelona'], ['galatasaray'], ['inter'], ['juventus', 'turin', 'juve'], ['genk'], ['lille', 'osc'],
      ['liverpool'], ['lokomotiv'], [2, 'manchester', 'man', 'city'], ['olympiakos', 'piräus', 'piraus', 'olympiacos'], ['olympique', 'lyon'], ['paris', 'saint', 'germain', 'psg'], ['leipzig'], ['salzburg'], ['real'], ['schachtar', 'donezk', 'shakhtar', 'donetsk'], ['benfica'], ['slavia', 'praha', 'prag'], ['ssc', 'neapel', 'napoli'], ['tottenham', 'hotspur'], ['valencia'], ['zenit', 'petersburg']]

prem = [['bournemouth'], ['arsenal'], ['aston', 'villa'], ['brighton'], ['burnley'], ['chelsea'], ['crystal', 'palace'], ['everton'], ['leicester'], ['liverpool'], [2, 'manchester', 'city', 'man'], [
    2, 'manchester', 'united', 'man'], ['newcastle'], ['norwich'], ['sheffield', 'sheff'], ['southampton'], ['tottenham', 'hotspur', 'spurs'], ['watford'], ['west', 'ham'], ['wolverhampton', 'wanderers']]


def setup(competition):
    if competition.lower() in ['buli', 'bundesliga', 'bl', 'german', 'germany']:
        subreddit = redditBot(secrets.buli_user_agent,
                              secrets.buli_client_id, secrets.buli_client_secret)
        chat_id = secrets.buli_chat_id
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp(buli)
        print(bcolors.OKGREEN + 'STARTED BUNDESLIGA BOT' + bcolors.ENDC)
        return setupObject(subreddit, chat_id, bot, comp)
    elif competition.lower() in ['cl', 'champions', 'championsleague', 'ucl']:
        subreddit = redditBot(secrets.cl_user_agent,
                              secrets.cl_client_id, secrets.cl_client_secret)
        chat_id = secrets.cl_chat_id
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp(cl)
        print(bcolors.OKGREEN + 'STARTED CHAMPIONS LEAGUE BOT' + bcolors.ENDC)
        return setupObject(subreddit, chat_id, bot, comp)
    elif competition.lower() in ['premier', 'premierleague', 'pl', 'prem', 'bpl']:
        subreddit = redditBot(secrets.prem_user_agent,
                              secrets.prem_client_id, secrets.prem_client_secret)
        chat_id = secrets.prem_chat_id
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp(prem)
        print(bcolors.OKGREEN + 'STARTED PREMIER LEAGUE BOT' + bcolors.ENDC)
        return setupObject(subreddit, chat_id, bot, comp)
    else:
        raise Exception('unkown league')


def redditBot(userAgent, clientId, clientSecret):
    reddit = praw.Reddit(
        user_agent=userAgent,
        client_id=clientId,
        client_secret=clientSecret
    )
    subreddit = reddit.subreddit('soccer')
    return subreddit


def createComp(league):
    teams = []
    for team in league:
        if isinstance(team[0], int):
            matchesNeeded = team.pop(0)
            tempTeam = Team(team, matchesNeeded)
        else:
            tempTeam = Team(team)
        teams.append(tempTeam)
    return Competition(teams)


class setupObject:
    def __init__(self, subreddit=None, chat_id=None, bot=None, competition=None):
        if not(subreddit and chat_id and bot and competition):
            raise Exception(
                'setupObejct constructor was called with missing arguments')
        self.subreddit = subreddit
        self.chat_id = chat_id
        self.bot = bot
        self.competition = competition


# import praw
# import secrets
# from competition import Competition, Team
# import telegram
# from bcolors import bcolors

# buli = [['gladbach', 'mönchengladbach', 'monchengladbach'], ['leipzig'], ['bayern', 'münchen', 'munchen', 'munich'], ['freiburg'], ['hoffenheim'], ['dortmund'], ['schalke'], ['leverkusen', 'bayer'],
#         ['frankfurt'], ['wolfsburg'], ['union'], ['hertha'], ['düsseldorf'], ['dusseldorf'], ['werder', 'bremen'], ['augsburg'], ['mainz'], ['köln', 'koln', 'cologne'], ['paderborn']]

# cl = [['ajax', 'amsterdam'], ['atalanta'], ['atlético', 'atletico'], ['bayer', 'leverkusen'], ['bayern', 'münchen', 'munich', 'munchen'], ['dortmund'], ['chelsea'], ['brügge', 'brugge'], ['roter', 'stern', 'belgrad', 'red', 'star'], ['dinamo', 'zagreb', 'dynamo'], ['barcelona'], ['galatasaray'], ['inter'], ['juventus', 'turin', 'juve'], ['genk'], ['lille', 'osc'],
#       ['liverpool'], ['lokomotiv'], [2, 'manchester', 'man', 'city'], ['olympiakos', 'piräus', 'piraus', 'olympiacos'], ['olympique', 'lyon'], ['paris', 'saint', 'germain', 'psg'], ['leipzig'], ['salzburg'], ['real'], ['schachtar', 'donezk', 'shakhtar', 'donetsk'], ['benfica'], ['slavia', 'praha', 'prag'], ['ssc', 'neapel', 'napoli'], ['tottenham', 'hotspur'], ['valencia'], ['zenit', 'petersburg']]

# prem = [['bournemouth'], ['arsenal'], ['aston', 'villa'], ['brighton'], ['burnley'], ['chelsea'], ['crystal', 'palace'], ['everton'], ['leicester'], ['liverpool'], [2, 'manchester', 'city', 'man'], [
#     2, 'manchester', 'united', 'man'], ['newcastle'], ['norwich'], ['sheffield', 'sheff'], ['southampton'], ['tottenham', 'hotspur', 'spurs'], ['watford'], ['west', 'ham'], ['wolverhampton', 'wanderers']]


# def redditBot():
#     reddit = praw.Reddit(
#         user_agent=secrets.reddit_user_agent,
#         client_id=secrets.reddit_client_id,
#         client_secret=secrets.reddit_client_secret
#     )
#     subreddit = reddit.subreddit('soccer')
#     return subreddit


# def telegramBot():
#     return telegram.Bot(token=secrets.telegram_token)


# def competition(competition):
#     teams = []
#     if competition.lower() in ['buli', 'bundesliga', 'bl', 'german', 'germany']:
#         print(bcolors.OKGREEN + 'STARTED BUNDESLIGA BOT' + bcolors.ENDC)
#         return createComp(buli)
#     elif competition.lower() in ['cl', 'champions', 'championsleague', 'ucl']:
#         print(bcolors.OKGREEN + 'STARTED CHAMPIONS LEAGUE BOT' + bcolors.ENDC)
#         return createComp(cl)
#     elif competition.lower() in ['premier', 'premierleague', 'pl', 'prem', 'bpl']:
#         print(bcolors.OKGREEN + 'STARTED PREMIER LEAGUE BOT' + bcolors.ENDC)
#         return createComp(prem)
#     else:
#         raise Exception


# def chat_id(competition):
#     if competition.lower() in ['buli', 'bundesliga', 'bl', 'german', 'germany']:
#         return secrets.buli_chat_id
#     elif competition.lower() in ['cl', 'champions', 'championsleague', 'ucl']:
#         return secrets.cl_chat_id
#     elif competition.lower() in ['premier', 'premierleague', 'pl', 'prem', 'bpl']:
#         return secrets.prem_chat_id
#     else:
#         raise Exception


# def createComp(league):
#     teams = []
#     for team in league:
#         if isinstance(team[0], int):
#             matchesNeeded = team.pop(0)
#             tempTeam = Team(team, matchesNeeded)
#         else:
#             tempTeam = Team(team)
#         teams.append(tempTeam)
#     return Competition(teams)