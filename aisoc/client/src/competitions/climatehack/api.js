import { request } from 'api/common';
import CompetitionAPI from 'api/competition';

export default class ClimateHackAPI extends CompetitionAPI {
  static COMPETITION_ID = 'climatehack';

  static async getActiveGameId(username) {
    const activeGames = await this.getUserActiveGames(username);
    if (activeGames && activeGames.length >= 1) {
      return activeGames[0].id;
    }

    return null;
  }

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

  static async reactivateAgent(agent, authToken) {
    await request({ url: this.AGENT_BASE_URL + agent + '/reactivate', method: 'POST', authToken });
  }
}
