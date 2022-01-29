// TODO: maybe move this to a `models` folder and include other models for representing responses from API

const UNIVERSITIES_DOMAIN_MAP = {
  'UNKNOWN': {
    name: 'Unknown',
    logo: null
  },
  'ucl.ac.uk': {
    name: 'UCL',
    logo: null
  },
  'warwick.ac.uk': {
    name: 'Warwick',
    logo: null
  },
  'gla.ac.uk': {
    name: 'Glasgow',
    logo: null
  },
  'princeton.edu': {
    name: 'Princeton',
    logo: null
  },
  'imperial.ac.uk': {
    name: 'Princeton',
    logo: null
  },
  'utoronto.ca': {
    name: 'Toronto',
    logo: null
  },
  'manchester.ac.uk': {
    name: 'Manchester',
    logo: null
  },
};

function buildUniversitiesTrie(unis) {
  let rootTrie = { trie: true };

  for (const uniDomain in unis) {
    const parts = uniDomain.split('.');
    let trie = rootTrie;

    for (let i = parts.length - 1; i >= 0; i--) {
      const domainPart = parts[i];

      if (!Object.prototype.hasOwnProperty.call(trie, domainPart)) {
        trie[domainPart] = {
          trie: true,
        };
      } else if (i == 0) {
        throw new Error('Overlapping uni domains');
      }

      trie = trie[domainPart];
    }

    trie.trie = false;
    trie.name = unis[uniDomain].name;
    trie.logo = unis[uniDomain].logo;
  }

  return rootTrie;
}

function findUniversity(rootTrie, uniDomain) {
  const parts = uniDomain.split('.');

  let trie = rootTrie;
  for (let i = parts.length - 1; i >= 0; i--) {
    const part = parts[i];

    if (!Object.prototype.hasOwnProperty.call(trie, part)) {
      return UNIVERSITIES_DOMAIN_MAP['UNKNOWN'];
    }

    trie = trie[part];
    // We've reached the leaf, this domain might be more specific but we stop at the first match
    if (!trie.trie) {
      return trie;
    }


  }

  console.error('Reached end of domain without finding specific university, this domain is too general', uniDomain);
  return UNIVERSITIES_DOMAIN_MAP['UNKNOWN'];
}

const UNIVERSITIES_TRIE = buildUniversitiesTrie(UNIVERSITIES_DOMAIN_MAP);

export default class UserProfile {
  constructor(profile) {
    this.profile = profile;

    this._university = findUniversity(UNIVERSITIES_TRIE, (profile.extra && profile.extra.org) || 'UNKNOWN');

  }

  name() {
    return this.profile.username;
  }

  university() {
    return this._university;
  }
}
