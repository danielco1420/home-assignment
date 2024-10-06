import Redis from 'ioredis';

const redis = new Redis({
    host: process.env.REDIS_HOST || 'localhost',
    port: process.env.REDIS_PORT || 6379
});

export const addToken = async (token) => {
    return await redis.hset('signedTokens', token, JSON.stringify({ wins: 0, losses: 0 }));
};

export const getToken = async (token) => {
    const result = await redis.hget('signedTokens', token);
    return result ? JSON.parse(result) : null;
};

export const getAllTokens = async () => {
    const result = await redis.hgetall('signedTokens');
    if (result) {
        for (const token in result) {
            result[token] = JSON.parse(result[token]);
        }
    }
    return result;
};

export const clearDB = async () => {
    const result = await redis.del('signedTokens') && await redis.del('totalWins');
    if (result) {
        for (const token in result) {
            result[token] = JSON.parse(result[token]);
        }
    }
    return result;
};

export const removeToken = async (token) => {
    return await redis.hdel('signedTokens', token);
};

export const getTotalWins = async () => {
    return await redis.get('totalWins')
}

export const tryLuck = async (token, didWin) => {
    const tokenData = await getToken(token);

    const totalWins = await redis.get('totalWins');

    if (didWin) {
        tokenData.wins += 1;
        await redis.set('totalWins', totalWins ? parseInt(totalWins) + 1 : 1);
    } else {
        tokenData.losses += 1;
    }

    const result = await redis.hset('signedTokens', token, JSON.stringify(tokenData));

    console.log("Updated Token Data:", tokenData);
    return { success: result };
};