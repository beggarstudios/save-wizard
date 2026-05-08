Save Wizard is a Rust-based TUI tool for generating save game files with lightweight game data management capabilities. The goal is to facilitate rapid testing for developers without the need of hardcoded dev portals or playing through the game to reach the desired game-state.

### Requirements

The app must support a generic entity system and custom schema validation. This allows the app to adapt to in-game data structures and give them context within the app.

Workflows to define game data manifests must be relatively effortless from a UX standpoint.

The app must support various types of input data te generate manifests from, starting with JSON input.

Users must be able to define their own save file schemas. Users should be able to configure everything in-app without touching Rust code

## Scope

Initial versions will support JSON schemas and local project-based workflows

Users must be able to:

- [ ] Set a path to their game data
- [ ] Convert game data to manifests
- [ ] Define entity types
- [ ] Define entity type schemas
- [ ] Define save game schemas
- [ ] Create entity records
- [ ] Generate save files from entity records

### Terminology

**Manifest**  
Defines game content and entity data available to Save Wizard.  
  
**Schema**  
Defines the structure and validation rules for save files or entity types.  

**Entity Type**
Defines the structure and fields of a category of entities, such as Bug, Move, Item, or Quest.
  
**Entity**  
A generic data object such as a bug, move, item, NPC, or quest.  
  
**Draft Save**  
A generated save-state instance built from manifest entities.