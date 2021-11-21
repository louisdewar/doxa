import api from '../../common/api';

export default class UTTTAPI extends api.CompetitionAPI {
  constructor() {
    super();

    this.AGENT_BASE_URL = this.BASE_URL + 'competition/uttt/_agent/';
    this.GAME_BASE_URL = this.BASE_URL + 'competition/uttt/_game/';
    this.LEADERBOARD_BASE_URL = this.BASE_URL + 'competition/uttt/_leaderboard/';
    this.USER_BASE_URL = this.BASE_URL + 'competition/uttt/_user/';
  }

  async getUTTTGameWinners(game) {
    const data = await this.getSingleGameEvent(game, 'game_winners');

    return data.payload.winners;
  }

  async getUTTTGameScores(game) {
    const data = await this.getSingleGameEvent(game, 'scores');

    return data.payload;
  }

  async getUTTTGameEvents(match, subGameID) {
    const data = await this.getSingleGameEvent(match, 'game_' + subGameID);

    return data.payload.events;
  }

}
