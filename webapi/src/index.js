import express from 'express';
import ClientsService from './services/clientService.js';

const app = express();
const port = 3000;

app.use(express.json());

const clientsService = new ClientsService();

app.get('/', (req, res) => {
    res.send('Hello World!');
});

app.post('/clients', async (req, res) => {
    const data = req.body;
    await clientsService.insertClients(data);
    res.send(data);
});

app.get('/clients', async (req, res) => {
    const data = await clientsService.getClients();
    res.json(data);
});

app.listen(port, () => {
    console.log(`Web API listening on port ${port}`);
});