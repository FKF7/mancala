import Express from 'express';
import Cors from 'cors';
// import UserRoutes from './routes/userRoutes.ts';
import UserRoutes from './routes/userRoutes.ts';
import MancalaRoutes from './routes/mancalaGameRoutes.ts';

// const express = require('express');
// const cors = require('cors');

// const productRoutes = require('./routes/productRoutes');
// const orderRoutes = require('./routes/orderRoutes');

const app = Express();
app.use(Cors());
app.use(Express.json());

app.use('/api/users', UserRoutes);
app.use('/api/mancala', MancalaRoutes);
// app.use('/api/products', productRoutes);
// app.use('/api/orders', orderRoutes);

app.listen(3001, () => console.log(`server running on http://localhost:3001`));
