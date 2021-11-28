import { request } from './common';


class CompetitionAPI {

  // configure in .env, must end with a '/'
  static BASE_URL = process.env.REACT_APP_API_BASE_URL;

  static AGENT_BASE_URL = null;
  static GAME_BASE_URL = null;
  static LEADERBOARD_BASE_URL = null;
  static USER_BASE_URL = null;

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

  static async getGameEvents(game, filter) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/events',
      method: 'GET',
      params: filter ? { t: filter } : null,
    });

    return data.events;
  }

  static async getSingleGameEvent(game, filter) {
    const events = await this.getGameEvents(game, filter);

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

export default { CompetitionAPI };
