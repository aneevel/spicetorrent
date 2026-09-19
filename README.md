# Kitchen Catalog Backend Server 

Backend Service for Kitchen / Pantry organizing application, suitable for keeping track of ingredients in household, as well as recipes including 
those recipes. Capable of making suggestions based on configurable parameters, including but not limited to available ingredients, specified nutrients, and flavor profiles. 

Users can perform standard CRUD operations on two models; "Ingredients" and "Recipes". Ingredients are the raw foodstuffs that we combine into 
meals and then eat, while Recipes are lists of Ingredients combined with text instructions and some metadata such as tags, total cooking time, and serving size. Most parameters are optional to allow users to focus on only what they care about and skip the rest. 

The service makes a distinction between ingredients in the "Catalog" or database versus ingredients present in the "Inventory". Any ingredient you've 
added to the service at all will exist in the Catalog, and while operations exist to remove it, use cases for doing so are limited. The Inventory is 
what you actually have on hand in your household, ready for use. This model allows you to make choices in real-time for what to cook right now based on how you answer questions like "Do I want to go to the store right now?", or "What do I need to get if I do go to the store?". The intention is to reduce the friction of figuring out what you want to cook at any given meal, and how you can get that.

The intention of this service is to run in a decentralized fashion so each household can manage their own records, and no user management or third-party
data management takes place. Data is stored in easily-readable and modifiable JSON files, avoiding propietary or obscure data formats. If an user wishes
to modify their data outside of the confines of Kitchen Catalog, they are encouraged to do so. 

This service can serve data through either public or private networks; a startup walkthrough script exists for initial configuration, but further configuration can be done through either startup scripts or modifying configuration files with the editor of your choice. Again, JSON format makes 
no assumptions that you will utilize Kitchen Catalog sources to handle configuration.

## Features 

Services are organized into three different domains by function; Inventory, Catalog, and Suggestions. While these domains are roughly how I think 
about doing different things, there is a bit of cross-functionality. For example, asking for a suggestion of a meal will go to the Suggestions endpoint, but behind the scenes this will require asking Inventory and Catalog services some questions about what they have available. 

A(n incomplete) list of features;
- Store available ingredients, including cost, purchase and expiration dates, amount, nutrient profile, etc.,
- Store recipes including list of ingredients and text instructions, along with metadata such as total cost, total cooking time, etc., 
- Provide suggestions based on parameters such as ingredients included, nutrient profile, and more 
- Store data in JSON 
- Easy to use startup script 
- Easy to modify configuration 
- Ability to run over private or public networks

## License 

Kitchen Catalog is open source software, and all components are released under the [MIT License](https://raw.githubusercontent.com/kitchen-catalog/LICENSE.txt).

## Contributing

I have no documentation currently available for contributions to this project, and no current intention to provide any; if you want to make any contributions, you're certainly welcome to open an issue or email me at <aneevel15@gmail.com>. I'll probably be a bit surprised to hear this to be 
frank, but please do not think that surprise is anything other than pleasant.
