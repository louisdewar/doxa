import api from '../../common/api';

export default class UTTTAPI extends api.CompetitionAPI {

  static AGENT_BASE_URL = this.BASE_URL + 'competition/uttt/_agent/';
  static GAME_BASE_URL = this.BASE_URL + 'competition/uttt/_game/';
  static LEADERBOARD_BASE_URL = this.BASE_URL + 'competition/uttt/_leaderboard/';
  static USER_BASE_URL = this.BASE_URL + 'competition/uttt/_user/';

  static async getUTTTGameWinners(game) {
    const data = await this.getSingleGameEvent(game, 'game_winners');

    return data.payload.winners;
  }

  static async getUTTTGameScores(game) {
    const data = await this.getSingleGameEvent(game, 'scores');

    return data.payload;
  }

  static async getUTTTGameEvents(match, subGameID) {
    const data = await this.getSingleGameEvent(match, 'game_' + subGameID);

    return data.payload.events;
  }

}
