import ClientRepository from "../repositories/clientRepository.js";

class ClientService {
    constructor() {
        this.clientRepository = new ClientRepository();
    }

    async getClients() {
        return await this.clientRepository.getClients();
    }

    async insertClients(clients) {
        await this.clientRepository.insertClients(clients);
    }
}

export default ClientService;