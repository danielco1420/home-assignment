import express from "express";
import { getAllTokens, addToken, removeToken, clearDB, tryLuck, getTotalWins } from "./utils/data.js";
import { generateNewToken, getUserToken } from "./utils/token.js";

const router = express.Router();

router.get('/', (_, res) => {
    res.send('Server is working');
})

router.get('/all', async (req, res) => {
    try {
        const tokens = await getAllTokens();
        res.status(200).send(tokens);
    } catch (error) {
        console.error("Error fetching tokens:", error);
        res.status(500).send("Internal Server Error");
    }
});

router.post('/api/login', async (req, res) => {
    try {
        const randomToken = generateNewToken(req);
        if (randomToken) {
            const success = await addToken(randomToken);
            if (success) {
                res.send({ token: randomToken });
            } else {
                res.status(500).send("Error saving token");
            }
        } else {
            res.status(401).send({ "error": "Wrong email and password" });
        }
    } catch (error) {
        console.log("Error in /login route:", error);
        res.status(500).send("Server error");
    }
})

router.post('/api/logout', async (req, res) => {
    try {
        const userToken = getUserToken(req);
        const success = await removeToken(userToken);
        if (success) {
            res.send(`"OK"`);
        } else {
            res.status(500).send("Error removing token");
        }
    } catch (error) {
        console.log("Error in /logout route:", error);
        res.status(500).send("Server error");
    }
})

router.post('/api/try_luck', async (req, res) => {
    try {
        const userToken = getUserToken(req);
        const totalWins = await getTotalWins();
        const didWin = totalWins > 30 ? Math.random() < 0.4 : Math.random() < 0.7
        const success = await tryLuck(userToken, didWin)
        if (success) {
            res.send({ "win": didWin })
        } else {
            res.status(500).send("Error trying");
        }
    } catch (error) {

    }
})

router.get('/total_wins', async (req, res) => {
    try {
        const totalWins = await getTotalWins();
        res.send({ totalWins })
    } catch (error) {

    }
})

router.delete('/', async (req, res) => {
    try {
        const success = await clearDB();
        if (success) {
            res.send("CLEARED");
        } else {
            res.status(500).send("Error clearing DB");
        }
    } catch (error) {
        console.log("Error in delete route:", error);
        res.status(500).send("Server error");
    }
})

export default router;