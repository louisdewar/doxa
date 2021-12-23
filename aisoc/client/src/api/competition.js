import { request } from './common';


export default class CompetitionAPI {

  // NOTE: REACT_APP_API_BASE_URL must end with a '/' and be properly configured in .env

  static COMPETITION_ID = null;

  static get AGENT_BASE_URL() {
    return `${process.env.REACT_APP_API_BASE_URL}competition/${this.COMPETITION_ID}/_agent/`;
  }

  static get GAME_BASE_URL() {
    return `${process.env.REACT_APP_API_BASE_URL}competition/${this.COMPETITION_ID}/_game/`;
  }

  static get LEADERBOARD_BASE_URL() {
    return `${process.env.REACT_APP_API_BASE_URL}competition/${this.COMPETITION_ID}/_leaderboard/`;
  }

  static get USER_BASE_URL() {
    return `${process.env.REACT_APP_API_BASE_URL}competition/${this.COMPETITION_ID}/_user/`;
  }

  /* Agent */

  static async getAgentGames(agent) {
    const data = await request({ url: this.AGENT_BASE_URL + agent + '/games', method: 'GET' });

    return data.games;
  }

  static async getAgentScore(agent) {
    const data = await request({ url: this.AGENT_BASE_URL + agent + '/score', method: 'GET' });

    return data;
  }

  /* Game */
  static async getGame(game) {
    const data = await request({
      url: this.GAME_BASE_URL + game,
      method: 'GET',
    });

    for (let field of ['completed_at', 'started_at', 'queued_at']) {
      if (data[field]) {
        data[field] = new Date(data[field]);
      }
    }

    return data;
  }

  static async getGameEvents(game, filter, authToken) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/events',
      method: 'GET',
      params: filter ? { t: filter } : undefined,
      authToken
    });

    return data.events;
  }

  static async getSingleGameEvent(game, filter, authToken) {
    const events = await this.getGameEvents(game, filter, authToken);

    return events[0];
  }

  static async getGamePlayers(game) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/players',
      method: 'GET',
    });

    return data.players;
  }

  static async getGameResult(game, agent) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/result/' + agent,
      method: 'GET',
    });

    return data.result;
  }

  /* Leaderboard */

  static async getLeaderboardActive() {
    const data = await request({ url: this.LEADERBOARD_BASE_URL + 'active', method: 'GET' });

    return data.leaderboard;
  }

  /* User */

  static async getUserActiveAgent(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/active_agent', method: 'GET' });

    return data.active_agent;
  }

  static async getUserScore(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/score', method: 'GET' });

    return data;
  }

  static async getUserActiveGames(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/active_games', method: 'GET' });

    return data.games;
  }

}
