class Competition:
    def __init__(self, team=None):
        if not(team is None):
            self.teams = []
            self.addTeam(team)
        else:
            self.teams = []

    def __str__(self):
        return str(self.teams)

    def addTeam(self, team):
        if isinstance(team, list):
            for t in team:
                if isinstance(t, Team): self.teams.append(t)
        else:
            if isinstance(team, Team): self.teams.append(team)

    def isCompetition(self, strings):
        indexOfHyphen = list('-' in e for e in strings).index(True)
        left = strings[:indexOfHyphen]
        right = strings[indexOfHyphen+1:]
        if any(team.isTeam(left) for team in self.teams) and any(team.isTeam(right) for team in self.teams):
            return True
        return False

class Team:
    def __init__(self, alias=None, matchesNeeded=1):
        self.matchesNeeded = matchesNeeded
        self.aliases = []
        if not(alias is None):
            self.addAlias(alias)
    
    def __str__(self):
        return str(aliases)

    def addAlias(self, alias):
        if isinstance(alias, list):
            self.aliases.extend(alias)
        elif isinstance(alias, str):
            self.aliases.append(alias)

    def isTeam(self, strings):
        if sum(alias in strings for alias in self.aliases) >= self.matchesNeeded:
            return True
        return False
