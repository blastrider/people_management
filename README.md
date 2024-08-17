# Documentation de l'API Utilisateurs

## Introduction

Cette API permet de gérer les utilisateurs dans un système. Elle offre les fonctionnalités suivantes :
- Création d'un utilisateur
- Modification d'un utilisateur existant
- Suppression d'un utilisateur

L'API est conçue pour être utilisée avec des requêtes HTTP standard.

## Endpoints

### 1. Créer un utilisateur

- **Endpoint** : `/api/users`
- **Méthode HTTP** : `POST`
- **Description** : Crée un nouvel utilisateur avec les informations fournies.

#### Corps de la requête

Le corps de la requête doit être au format JSON et contenir les champs suivants :

```json
{
  "first_name": "string",
  "last_name": "string",
  "password_hash": "string",
  "address": "string",
  "phone_number": "string",
  "ssn": "string"
}
```

- `first_name` : Prénom de l'utilisateur (obligatoire).
- `last_name` : Nom de l'utilisateur (obligatoire).
- `password_hash` : Hachage du mot de passe (obligatoire).
- `address` : Adresse de l'utilisateur (facultatif).
- `phone_number` : Numéro de téléphone de l'utilisateur (facultatif).
- `ssn` : Numéro de sécurité sociale (facultatif).

#### Exemple de requête

```bash
curl -X POST http://localhost:8000/api/users \
-H "Content-Type: application/json" \
-d '{
  "first_name": "John",
  "last_name": "Doe",
  "password_hash": "hashed_password_example",
  "address": "123 Main St, Springfield",
  "phone_number": "+123456789",
  "ssn": "123-45-6789"
}'
```

#### Réponse

- **Code 200** : Utilisateur créé avec succès.
  - Corps de la réponse : Détails de l'utilisateur créé (au format JSON).
- **Code 400** : Erreur de requête (par exemple, champ manquant ou mal formaté).
  - Corps de la réponse : Message d'erreur.

### 2. Modifier un utilisateur

- **Endpoint** : `/api/users/<user_id>`
- **Méthode HTTP** : `PUT`
- **Description** : Met à jour les informations d'un utilisateur existant.

#### Paramètres

- `user_id` : ID de l'utilisateur à mettre à jour (dans l'URL).

#### Corps de la requête

Le corps de la requête doit être au format JSON et peut contenir un ou plusieurs des champs suivants :

```json
{
  "first_name": "string",
  "last_name": "string",
  "password_hash": "string",
  "address": "string",
  "phone_number": "string",
  "ssn": "string"
}
```

- Tous les champs sont facultatifs, et seuls les champs présents dans la requête seront mis à jour.

#### Exemple de requête

```bash
curl -X PUT http://localhost:8000/api/users/1 \
-H "Content-Type: application/json" \
-d '{
  "first_name": "Jane",
  "address": "456 Elm St, Springfield"
}'
```

#### Réponse

- **Code 200** : Utilisateur mis à jour avec succès.
  - Corps de la réponse : Détails de l'utilisateur mis à jour (au format JSON).
- **Code 400** : Erreur de requête (par exemple, ID de l'utilisateur invalide).
  - Corps de la réponse : Message d'erreur.
- **Code 404** : Utilisateur non trouvé.

### 3. Supprimer un utilisateur

- **Endpoint** : `/api/users/<user_id>`
- **Méthode HTTP** : `DELETE`
- **Description** : Supprime un utilisateur existant.

#### Paramètres

- `user_id` : ID de l'utilisateur à supprimer (dans l'URL).

#### Exemple de requête

```bash
curl -X DELETE http://localhost:8000/api/users/1
```

#### Réponse

- **Code 204** : Utilisateur supprimé avec succès. Aucun contenu n'est renvoyé dans la réponse.
- **Code 400** : Erreur de requête (par exemple, ID de l'utilisateur invalide).
  - Corps de la réponse : Message d'erreur.
- **Code 404** : Utilisateur non trouvé.

## Erreurs Courantes

- **400 Bad Request** : Cette erreur se produit si la requête est mal formée, par exemple, si un champ obligatoire est manquant ou mal formaté.
- **404 Not Found** : Cette erreur se produit si l'ID de l'utilisateur fourni dans l'URL n'existe pas.

## Conclusion

Cette API offre un moyen simple de gérer les utilisateurs au sein d'une application. Vous pouvez utiliser des outils comme `curl` ou des clients HTTP pour interagir avec l'API. Assurez-vous de toujours envoyer des données bien formatées pour éviter les erreurs.

