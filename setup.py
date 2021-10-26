import praw
from competition import Competition, Team
import telegram
import json
import sys

supported_competitions = ['buli', 'cl', 'prem', 'internationals', 'formula1']


def setup():
  # When no league is passed, bundesliga is selected.
  comp_title = 'buli' if len(sys.argv) is 1 else sys.argv[1]
  comp_title = comp_title.lower()

  if comp_title not in supported_competitions:
    raise Exception(f'Unkown league. Use one of these: {supported_competitions}')

  secrets = read_secrets()
  bot = telegram.Bot(token=secrets["telegram_token"])
  comp = create_comp(comp_title)

  apis = {
    "bot": bot,
    "competition": comp,
    "chat_id": secrets[f'{comp_title}_chat_id'],
    "reddit": praw.Reddit(
      user_agent=secrets[f'reddit_user_agent'],
      client_id=secrets[f'reddit_client_id'],
      client_secret=secrets[f'reddit_client_secret']
    ),
    "subreddit": praw.Reddit(
      user_agent=secrets[f'reddit_user_agent'],
      client_id=secrets[f'reddit_client_id'],
      client_secret=secrets[f'reddit_client_secret']
    ).subreddit('formula1' if comp_title == 'formula1' else 'soccer')
  }
  print(f'STARTED {comp_title.upper()} BOT')
  return apis


def create_comp(comp):
  with open(f'./competitions/{comp}.json') as comp_json:
    comp_dict = json.load(comp_json)
    teams = []
    for team in comp_dict["teams"]:
      matches_needed = team["min_matches"]
      temp_team = Team(team["names"], matches_needed)
      teams.append(temp_team)
  return Competition(teams)


def read_secrets():
  with open('./secrets.json') as secrets:
    return json.load(secrets)
