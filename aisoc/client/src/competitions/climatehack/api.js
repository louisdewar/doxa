import { request } from 'api/common';
import CompetitionAPI from 'api/competition';

export default class ClimateHackAPI extends CompetitionAPI {
  static COMPETITION_ID = 'climatehack';

  static async getLeaderboard(leaderboard) {
    const data = await request({ url: `${this.LEADERBOARD_BASE_URL}active/${leaderboard}`, method: 'GET' });

    return data.leaderboard;
  }
}
