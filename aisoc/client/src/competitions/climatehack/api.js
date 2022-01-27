import CompetitionAPI from 'api/competition';

export default class ClimateHackAPI extends CompetitionAPI {
  static COMPETITION_ID = 'climatehack';

  static async getActiveGameEvents(username, authToken) {
    const activeGames = await this.getUserActiveGames(username);
    if (activeGames && activeGames.length >= 1) {
      const gameEvents = await this.getGameEvents(activeGames[0].id, undefined, authToken);
      if (gameEvents) {
        return gameEvents;
      }
    }

    return null;
  }
}
