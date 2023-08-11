from astron.object_repository import ClientRepository
from direct.showbase.ShowBase import ShowBase
from direct.task import Task
from globals import *
import sys


class SimpleClient(ShowBase):
    def __init__(self):
        # Basics
        ShowBase.__init__(self)
        self.disableMouse()
        self.accept("escape", self.disconnect)
        self.notify.setInfo(True)
        self.camera.set_pos(0, 0, 60)
        self.camera.look_at(0, 0, 0)
        # Game-relevant attributes
        self.avatar_ov = None  # once received, is of class `DistributedAvatarOV`
        self.avatar_ready = False
        # Avatar controls
        # FIXME: These values will be off the kilter if keys are pressed when the client starts.
        self.movement_heading = 0
        self.movement_speed = 0
        self.accept("avatar_ov", self.get_avatar)
        self.accept("distributed_avatar", self.get_distributed_avatar)
        self.accept("w", self.indicate_movement, [0, 1])
        self.accept("w-up", self.indicate_movement, [0, -1])
        self.accept("s", self.indicate_movement, [0, -1])
        self.accept("s-up", self.indicate_movement, [0, 1])
        self.accept("a", self.indicate_movement, [1, 0])
        self.accept("a-up", self.indicate_movement, [-1, 0])
        self.accept("d", self.indicate_movement, [-1, 0])
        self.accept("d-up", self.indicate_movement, [1, 0])
        # TODO: Callback events. These names are "magic" (defined in AstronClientRepository)
        """
        self.accept("CLIENT_OBJECT_LEAVING", self.avatar_leaves)
        self.accept("CLIENT_OBJECT_LEAVING_OWNER", self.avatar_leaves_owner)
        self.accept("LOST_CONNECTION", self.lost_connection)
        """

        self.repo = ClientRepository(VERSION_STRING, DC_FILE)
        self.notify.info("Connecting...")
        self.repo.connect(self.connection_success, self.connection_failure, self.connection_eject,
                          host=CA_HOST, port=CA_PORT)
        # set task to poll datagrams every frame
        self.task_mgr.add(self.poll_datagrams, 'poll datagrams')

    def poll_datagrams(self, task):
        self.repo.poll_till_empty()
        return Task.cont

    #
    # Connection management (callbacks and helpers)
    #

    # Connection established. Send CLIENT_HELLO to progress from NEW to UNKNOWN.
    # Normally, there could be code here for things to do before entering making
    # the connection and actually interacting with the server.
    def connection_success(self):
        self.notify.info("Connected!")
        self.client_is_handshaked()

    def connection_failure(self):
        self.notify.error("Failed to connect")
        sys.exit()

    def connection_eject(self, a, b):  # FIXME: Use arguments received
        self.notify.info("Client ejected!")
        sys.exit()

    def lost_connection(self):
        self.notify.error("Lost connection.")
        sys.exit()

    # Voluntarily end the connection.
    def disconnect(self):
        self.repo.send_CLIENT_DISCONNECT()
        sys.exit()

    # Client has received CLIENT_HELLO_RESP and now is in state UNKNOWN.
    def client_is_handshaked(self):
        anonymous_contact = self.repo.create_distobjglobal_view("AnonymousContact", AnonymousContactID)
        # Attach map to scene graph
        self.map = self.loader.load_model("./resources/map.egg")
        self.map.reparent_to(self.render)
        # Log in and receive; leads to enter_owner (ownership of avatar)
        anonymous_contact.login("guest", "guest")

    def avatar_leaves(self, do_id):
        self.notify.info("Avatar leaving: " + str(do_id))

    def avatar_leaves_owner(self, do_id):
        self.notify.info("AvatarOV leaving: " + str(do_id))

    #
    # Interface
    #

    # Adjust current intention and send it.
    def indicate_movement(self, heading, speed):
        if self.avatar_ov and self.avatar_ready:
            # FIXME: Not really graceful to just ignore this.
            # What if a button was already pressed when we got the OV?

            self.movement_heading += heading
            self.movement_speed += speed
            self.avatar_ov.indicate_intent(self.movement_heading, self.movement_speed)
        else:
            print("Avatar not complete yet!")

    # A DistributedAvatarOV was created, here is it.
    def get_avatar(self, owner_view):
        print("Received DistributedAvatarOV in client")
        self.avatar_ov = owner_view
        self.taskMgr.add(self.complete_avatar, 'complete avatar')

    def complete_avatar(self, task):
        try:
            self.avatar_ready = True
            return Task.done
        except KeyError:
            print("Couldn't complete DistributedAvatarOV " + str(self.avatar_ov.do_id) + ", Available DOs: ".join(
                [str(self.avatar_ov.do_id) for self.avatar_ov.do_id in self.repo.distributed_objects.keys()]))
            return Task.cont

    # A DistributedAvatar was created, here is it.
    def get_distributed_avatar(self, avatar):
        print("Received DistributedAvatar " + str(avatar.do_id))


if __name__ == "__main__":
    simple_client = SimpleClient()
    simple_client.run()