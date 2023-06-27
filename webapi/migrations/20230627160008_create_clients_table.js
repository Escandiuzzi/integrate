/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
export function up(knex) {
    return knex.schema
        .createTable('clients', function (table) {
            table.increments('id').primary();
            table.string('name', 50).notNullable();
            table.integer('age').notNullable();
            table.string('address', 100).notNullable();
            table.integer('items_purchased');
        });
}

/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
export function down(knex) {
    return knex.schema
        .dropTable("clients");
}
