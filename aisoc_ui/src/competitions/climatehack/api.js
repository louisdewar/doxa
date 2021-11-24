import api from 'common/api';

export default class ClimateHackAPI extends api.CompetitionAPI {
  constructor() {
    super();

    this.AGENT_BASE_URL = this.BASE_URL + 'competition/climatehack/_agent/';
    this.GAME_BASE_URL = this.BASE_URL + 'competition/climatehack/_game/';
    this.LEADERBOARD_BASE_URL = this.BASE_URL + 'competition/climatehack/_leaderboard/';
    this.USER_BASE_URL = this.BASE_URL + 'competition/climatehack/_user/';
  }
}
