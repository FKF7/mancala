const PORT = 3001;

const Routes = Object.freeze({
    PORT,
    BASE_URL: `http://localhost:${PORT}`,
    API_URL: {
        USERS: '/api/users',
        PRODUCTS: '/api/products',
        ORDERS: '/api/orders',
        MANCALA: '/api/mancala',
        CJF: '/api/cjf',
    },
    ENDPOINTS: {
        USERS: {
            GET_ALL: '/getAllUsers',
            GET_BY_ID: '/getUserById',
        },
        MANCALA: {
            MAKE_MOVE: '/make_move',
            SIMULATE_GAME: '/simulate',
        },
    }
});

export default Routes;