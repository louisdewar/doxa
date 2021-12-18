import CompetitionAPI from 'api/competition';

export default class UTTTAPI extends CompetitionAPI {

  static COMPETITION_ID = 'uttt';

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
