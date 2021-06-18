class Competition:
  def __init__(self, team=None):
    if not (team is None):
      self.teams = []
      self.add_team(team)
    else:
      self.teams = []

  def __str__(self):
    return str(self.teams)

  def add_team(self, team):
    if isinstance(team, list):
      for t in team:
        if isinstance(t, Team): self.teams.append(t)
    else:
      if isinstance(team, Team): self.teams.append(team)

  def is_competition(self, strings):
    if len(self.teams) == 0:
      return True  # No Teams means wildcard
    index_of_hyphen = list('-' in e for e in strings).index(True)
    left = strings[:index_of_hyphen]
    right = strings[index_of_hyphen + 1:]
    if any(team.is_team(left) for team in self.teams) and any(team.is_team(right) for team in self.teams):
      return True
    return False


class Team:
  def __init__(self, alias=None, matches_needed=1):
    self.matchesNeeded = matches_needed
    self.aliases = []
    if not (alias is None):
      self.add_alias(alias)

  def __str__(self):
    return str(self.aliases)

  def add_alias(self, alias):
    if isinstance(alias, list):
      self.aliases.extend(alias)
    elif isinstance(alias, str):
      self.aliases.append(alias)

  def is_team(self, strings):
    if sum(alias in strings for alias in self.aliases) >= self.matchesNeeded:
      return True
    return False
