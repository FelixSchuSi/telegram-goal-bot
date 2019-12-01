class Competition:
    def __init__(self, team=None):
        if not(team is None):
            self.teams = []
            self.addTeam(team)
        else:
            self.teams = []

    def __str__(self):
        return str(teams)

    def addTeam(self, team):
        if isinstance(team, list):
            for t in team:
                if isinstance(t, Team): self.teams.append(t)
        else:
            if isinstance(team, Team): self.teams.append(team)

    def isCompetition(self, strings):
        if sum(team.isTeam(strings) for team in self.teams) >= 2:
            return True
        return False

class Team:
    def __init__(self, alias=None):
        if not(alias is None):
            self.aliases = []
            self.addAlias(alias)
        else:
            self.aliases = []
    
    def __str__(self):
        return str(aliases)

    def addAlias(self, alias):
        if isinstance(alias, list):
            self.aliases.extend(alias)
        elif isinstance(alias, str):
            self.aliases.append(alias)

    def isTeam(self, strings):
        if any(alias in strings for alias in self.aliases):
            return True
        return False
