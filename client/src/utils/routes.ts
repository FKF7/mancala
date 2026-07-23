const PORT = 3001;

const Routes = Object.freeze({
  PORT,
  BASE_URL: `http://localhost:${PORT}`,
  API_URL: "/api/mancala",
  ENDPOINTS: {
    MAKE_MOVE: "/make_move",
    GET_HINT_DATA: "/get_hint_data",
    SIMULATE_GAME: "/simulate",
    PRINT_PATH: "/print_path",
    PRINT_SEQUENCE: "/print_sequence",
    DECODE_CODE: "/decode_code",
  },
});

export default Routes;
