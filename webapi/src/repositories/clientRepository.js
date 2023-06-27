import knex from 'knex';

class ClientRepository {
    client = knex({
        client: 'sqlite3',
        connection: {
            filename: "./dev.sqlite3"
        },
        useNullAsDefault: true
    });


    async getClients() {
        let data;
        await this.client('clients')
            .timeout(1000)
            .then(result => {
                data = result;
            });

        return data;
    }

    async insertClients(clients) {
        try {
            await this.client('clients').insert(clients);
        } catch (ex) {
            console.log(ex);
        }
    }
}

export default ClientRepository;