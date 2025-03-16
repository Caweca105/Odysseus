import graphene
from graphene_django import DjangoObjectType
from .models import User

class UserType(DjangoObjectType):
    class Meta:
        model = User
        fields = ("id", "username", "email", "age", "comment", "location", "name", "preferences")

# For creating a user
class NewUserInput(graphene.InputObjectType):
    username = graphene.String(required=True)
    email = graphene.String(required=True)
    age = graphene.Int()
    comment = graphene.String()
    location = graphene.String()
    name = graphene.String()
    preferences = graphene.String()

class CreateUser(graphene.Mutation):
    user = graphene.Field(UserType)

    class Arguments:
        input = NewUserInput(required=True)

    def mutate(self, info, input):
        user = User.objects.create(
            username=input.username,
            email=input.email,
            age=input.age,
            comment=input.comment,
            location=input.location,
            name=input.name,
            preferences=input.preferences
        )
        return CreateUser(user=user)

# For deleting a user
class DeleteUser(graphene.Mutation):
    ok = graphene.Boolean()

    class Arguments:
        id = graphene.Int(required=True)

    def mutate(self, info, id):
        try:
            user = User.objects.get(pk=id)
            user.delete()
            return DeleteUser(ok=True)
        except User.DoesNotExist:
            return DeleteUser(ok=False)

# For updating a user
class UpdateUserInput(graphene.InputObjectType):
    id = graphene.Int(required=True)
    username = graphene.String()
    email = graphene.String()
    age = graphene.Int()
    comment = graphene.String()
    location = graphene.String()
    name = graphene.String()
    preferences = graphene.String()

class UpdateUser(graphene.Mutation):
    user = graphene.Field(UserType)
    ok = graphene.Boolean()

    class Arguments:
        input = UpdateUserInput(required=True)

    def mutate(self, info, input):
        try:
            user = User.objects.get(pk=input.id)
        except User.DoesNotExist:
            raise Exception("User not found")

        if input.username is not None:
            user.username = input.username
        if input.email is not None:
            user.email = input.email
        if input.age is not None:
            user.age = input.age
        if input.comment is not None:
            user.comment = input.comment
        if input.location is not None:
            user.location = input.location
        if input.name is not None:
            user.name = input.name
        if input.preferences is not None:
            user.preferences = input.preferences
        user.save()

        return UpdateUser(user=user, ok=True)

# Finally, wire them into the root Mutation
class Mutation(graphene.ObjectType):
    create_user = CreateUser.Field()
    update_user = UpdateUser.Field()
    delete_user = DeleteUser.Field()

class Query(graphene.ObjectType):
    users = graphene.List(UserType)

    def resolve_users(self, info):
        return User.objects.all()

schema = graphene.Schema(query=Query, mutation=Mutation)