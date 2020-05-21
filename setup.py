import praw
import secrets
from competition import Competition, Team
import telegram
import json

def setup(competition):
    if competition.lower() in ['buli', 'bundesliga', 'bl', 'german', 'germany']:
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp("buli")
        print('STARTED BUNDESLIGA BOT')
        return setupObject(secrets.buli_user_agent,
                              secrets.buli_client_id, secrets.buli_client_secret, secrets.buli_chat_id, bot, comp)
    elif competition.lower() in ['cl', 'champions', 'championsleague', 'ucl']:
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp("cl")
        print('STARTED CHAMPIONS LEAGUE BOT')
        return setupObject(secrets.cl_user_agent,
                              secrets.cl_client_id, secrets.cl_client_secret, secrets.cl_chat_id, bot, comp)
    elif competition.lower() in ['premier', 'premierleague', 'pl', 'prem', 'bpl','english', 'england']:
        bot = telegram.Bot(token=secrets.telegram_token)
        comp = createComp("prem")
        print('STARTED PREMIER LEAGUE BOT')
        return setupObject(secrets.prem_user_agent,
                              secrets.prem_client_id, secrets.prem_client_secret, secrets.prem_chat_id, bot, comp)
    else:
        raise Exception('unkown league')

def createComp(comp):
    with open(f'./competitions/{comp}.json') as compJson:
        compDict = json.load(compJson)
        teams = []
        for team in compDict["teams"]:
            matchesNeeded = team["min_matches"]
            tempTeam = Team(team["names"], matchesNeeded)
            teams.append(tempTeam)
    return Competition(teams)

class setupObject:
    def __init__(self, user_agent, client_id, client_secret, chat_id, bot, competition):
        self.subreddit = praw.Reddit(
            user_agent=user_agent,
            client_id=client_id,
            client_secret=client_secret
        ).subreddit('soccer')
        self.chat_id = chat_id
        self.bot = bot
        self.competition = competition