import { BASE_URL, request } from './common';

class CompetitionAPI {

  constructor() {
    this.BASE_URL = BASE_URL;

    // this.AGENT_BASE_URL = null;
    this.GAME_BASE_URL = null;
    this.LEADERBOARD_BASE_URL = null;
    this.USER_BASE_URL = null;
  }

  /* Agent */

  async getAgentGames(agent) {
    const data = await request({ url: this.AGENT_BASE_URL + agent + '/games', method: 'GET' });

    return data.games;
  }

  async getAgentScore(agent) {
    const data = await request({ url: this.AGENT_BASE_URL + agent + '/score', method: 'GET' });

    return data;
  }

  /* Game */

  async getGameEvents(game, filter) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/events',
      method: 'GET',
      params: filter ? { t: filter } : null,
    });

    return data.events;
  }

  async getSingleGameEvent(game, filter) {
    const events = await this.getGameEvents(game, filter);

    return events[0];
  }

  async getGamePlayers(game) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/players',
      method: 'GET',
    });

    return data.players;
  }

  async getGameResult(game, agent) {
    const data = await request({
      url: this.GAME_BASE_URL + game + '/result/' + agent,
      method: 'GET',
    });

    return data.result;
  }

  /* Leaderboard */

  async getLeaderboardActive() {
    const data = await request({ url: this.LEADERBOARD_BASE_URL + 'active', method: 'GET' });

    return data.leaderboard;
  }

  /* User */

  async getUserActiveAgent(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/active_agent', method: 'GET' });

    return data.active_agent;
  }

  async getUserScore(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/score', method: 'GET' });

    return data;
  }

  async getUserActiveGames(username) {
    const data = await request({ url: this.USER_BASE_URL + username + '/active_games', method: 'GET' });

    return data.games;
  }

}

export default { CompetitionAPI };
