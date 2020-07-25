import praw
from competition import Competition, Team
import telegram
import json
import sys

supportedCompetitions = ['buli', 'cl', 'prem']

def setup():
    # When no league is passed, the bundesliga bot is started.
    compTitle = 'buli' if len(sys.argv) is 1 else sys.argv[1]
    compTitle = compTitle.lower()

    if compTitle not in supportedCompetitions:
        raise Exception(f'Unkown league. Use one of these: {supportedCompetitions}')

    secrets = readSecrets()
    bot = telegram.Bot(token=secrets["telegram_token"])
    comp = createComp(compTitle)
    
    apis = {
        "bot": bot,
        "competition": comp,
        "chat_id": secrets[f'{compTitle}_chat_id'],
        "subreddit": praw.Reddit(
            user_agent=secrets[f'{compTitle}_user_agent'],
            client_id=secrets[f'{compTitle}_client_id'],
            client_secret=secrets[f'{compTitle}_client_secret']
        ).subreddit('soccer')        
    }

    print(f'STARTED {compTitle.upper()} BOT')

    return apis

def createComp(comp):
    with open(f'./competitions/{comp}.json') as compJson:
        compDict = json.load(compJson)
        teams = []
        for team in compDict["teams"]:
            matchesNeeded = team["min_matches"]
            tempTeam = Team(team["names"], matchesNeeded)
            teams.append(tempTeam)
    return Competition(teams)

def readSecrets():
    with open('./secrets.json') as secrets:
        return json.load(secrets)