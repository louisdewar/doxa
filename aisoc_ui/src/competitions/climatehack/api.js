import api from 'common/api';

export default class ClimateHackAPI extends api.CompetitionAPI {
  static AGENT_BASE_URL = this.BASE_URL + 'competition/climatehack/_agent/';
  static GAME_BASE_URL = this.BASE_URL + 'competition/climatehack/_game/';
  static LEADERBOARD_BASE_URL = this.BASE_URL + 'competition/climatehack/_leaderboard/';
  static USER_BASE_URL = this.BASE_URL + 'competition/climatehack/_user/';
}
