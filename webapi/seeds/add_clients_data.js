/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> } 
 */
export async function seed(knex) {
  // Deletes ALL existing entries
  await knex('clients').del()
  await knex('clients').insert([
    { name: 'Cristiano Ronaldo', age: 38, address: 'Aveiros - Portugal', items_purchased: 7 },
    { name: 'Neymar Jr.', age: 31, address: 'Santos - Brasil', items_purchased: 10 }
  ]);
}
