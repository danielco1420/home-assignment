const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const systemPassword = 'r2isthebest';

export const generateNewToken = (request) => {
    const email = String(request.body.email).trim();
    const password = String(request.body.password).trim();

    const isEmailValid = emailRegex.test(email);
    const isPasswordValid = password === systemPassword;

    if (isEmailValid && isPasswordValid) {
        const token = Math.floor(Math.random() * 1000000000);
        return token;
    } else {
        console.log("Validation failed");
        return false;
    }
};

export const getUserToken = (request) => {
    const authHeader = request.headers.authorization;

    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        console.log("Authorization header is missing or not valid");
        return false;
    }

    return authHeader.split('Bearer ')[1] || false;
};